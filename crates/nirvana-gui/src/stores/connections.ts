import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import {
  ConnectionSetupStep,
  ConnectionType,
  CreateConnectionInput,
  GuiConnection,
} from "../types/types";

export const connectionTypeLabels: Record<ConnectionType, string> = {
  "jira-cloud": "Jira Cloud",
  "jira-dc": "Jira Data Center",
};

export const normalizeHostname = (value: string) => {
  const trimmed = value.trim();
  const withoutProtocol = trimmed
    .replace(/^https?:\/\//i, "")
    .replace(/\/+$/g, "");
  return withoutProtocol;
};

export const useConnectionsStore = defineStore("connections", {
  state: () => ({
    activeConnection: null as GuiConnection | null,
    initialized: false,
    loading: false,
    error: "",
    setupStep: "details" as ConnectionSetupStep,
    draftType: "jira-cloud" as ConnectionType,
    draftName: "",
    draftHostname: "",
    draftUsername: "",
    draftToken: "",
  }),
  getters: {
    normalizedDraftHostname(state) {
      return normalizeHostname(state.draftHostname);
    },
    isDetailsValid(): boolean {
      return Boolean(
        this.draftType &&
          this.draftName.trim() &&
          this.normalizedDraftHostname,
      );
    },
    isCredentialsValid(state): boolean {
      return Boolean(state.draftUsername.trim() && state.draftToken.trim());
    },
    identityLabel(state): string {
      return state.draftType === "jira-cloud" ? "Email" : "Username";
    },
    connectionTypeLabel(state): string {
      return connectionTypeLabels[state.draftType];
    },
  },
  actions: {
    async initialize() {
      this.loading = true;
      this.error = "";

      try {
        this.activeConnection =
          await invoke<GuiConnection | null>("get_active_connection");
      } catch (error) {
        this.error =
          error instanceof Error ? error.message : String(error);
      } finally {
        this.loading = false;
        this.initialized = true;
      }
    },
    setConnectionType(type: ConnectionType) {
      this.draftType = type;
    },
    nextSetupStep() {
      if (!this.isDetailsValid) return;
      this.error = "";
      this.draftName = this.draftName.trim();
      this.draftHostname = this.normalizedDraftHostname;
      this.setupStep = "credentials";
    },
    previousSetupStep() {
      this.error = "";
      this.setupStep = "details";
    },
    async saveConnection() {
      if (!this.isDetailsValid || !this.isCredentialsValid) return false;

      this.loading = true;
      this.error = "";

      const input: CreateConnectionInput = {
        name: this.draftName.trim(),
        type: this.draftType,
        hostname: this.normalizedDraftHostname,
        username: this.draftUsername.trim(),
        token: this.draftToken.trim(),
      };

      try {
        this.activeConnection = await invoke<GuiConnection>("create_connection", {
          input,
        });
        this.draftToken = "";
        return true;
      } catch (error) {
        this.error =
          error instanceof Error ? error.message : String(error);
        return false;
      } finally {
        this.loading = false;
      }
    },
    resetSetup() {
      this.setupStep = "details";
      this.draftType = "jira-cloud";
      this.draftName = "";
      this.draftHostname = "";
      this.draftUsername = "";
      this.draftToken = "";
      this.error = "";
    },
  },
});
