export const formatDateTimeInput = (date: Date) => {
  const offsetMs = date.getTimezoneOffset() * 60 * 1000;
  return new Date(date.getTime() - offsetMs).toISOString().slice(0, 16);
};

export const parseDateTimeInput = (value: string) => {
  const date = new Date(value);
  return Number.isNaN(date.getTime()) ? null : date;
};

export type TimeParts = {
  hours: number;
  minutes: number;
  seconds: number;
};

export const formatTimeInput = (date: Date) =>
  `${date.getHours().toString().padStart(2, "0")}:${date
    .getMinutes()
    .toString()
    .padStart(2, "0")}`;

export const parseTimeParts = (value: string): TimeParts | null => {
  const match = value.trim().match(/^(\d{1,2}):(\d{2})(?::(\d{2}))?$/);
  if (!match) return null;

  const hours = Number(match[1]);
  const minutes = Number(match[2]);
  const seconds = match[3] ? Number(match[3]) : 0;

  if (
    !Number.isInteger(hours) ||
    !Number.isInteger(minutes) ||
    !Number.isInteger(seconds) ||
    hours < 0 ||
    hours > 23 ||
    minutes < 0 ||
    minutes > 59 ||
    seconds < 0 ||
    seconds > 59
  ) {
    return null;
  }

  return { hours, minutes, seconds };
};

export const formatTimeParts = (hours: number, minutes: number) =>
  `${hours.toString().padStart(2, "0")}:${minutes
    .toString()
    .padStart(2, "0")}`;

export const wrapTimePart = (value: number, maxExclusive: number) =>
  ((value % maxExclusive) + maxExclusive) % maxExclusive;

export const applyTimeParts = (date: Date, time: TimeParts) => {
  const nextDate = new Date(date);
  nextDate.setHours(time.hours, time.minutes, time.seconds, 0);
  return nextDate;
};
