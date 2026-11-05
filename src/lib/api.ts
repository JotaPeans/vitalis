import { invoke, type InvokeArgs } from "@tauri-apps/api/core";

import type { Command } from "./constants";

export type ResponseProps<T> =
  | { data: T; error: null }
  | { data: null; error: { message: string } };

export const api = {
  call: <T>(command: Command, args?: InvokeArgs) => invoke<T>(command, args),
};

function extractMessage(error: unknown) {
  if (typeof error === "string") return error;
  if (error && typeof error === "object") {
    const record = error as Record<string, unknown>;
    if (typeof record.error === "string") return record.error;
    if (typeof record.message === "string") return record.message;
  }
  return "Erro inesperado";
}

export async function createWrapQuery<T>(
  callback: () => Promise<T>,
  caller: string,
): Promise<ResponseProps<T>> {
  try {
    const data = await callback();
    return { data, error: null };
  } catch (error) {
    const message = extractMessage(error);
    console.error(`~ ${caller} error:`, message);
    return { data: null, error: { message } };
  }
}
