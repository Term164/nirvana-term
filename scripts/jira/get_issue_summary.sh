#!/bin/bash

source .env

curl --request GET \
    --url "${JIRA_BASE_URL}/rest/api/3/issue/DEV-1?fields=summary" \
    --user "${JIRA_USER_EMAIL}:${JIRA_CLOUD_API}" \
    --header 'Accept: application/json' | jq
