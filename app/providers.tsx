"use client";

import { ReactNode } from "react";
import { ClientProvider } from "@solana/react";
import { solanaClient } from "@/lib/solana-client";

export function Providers({ children }: { children: ReactNode }) {
  return <ClientProvider client={solanaClient}>{children}</ClientProvider>;
}
