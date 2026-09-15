import type { Metadata } from "next";
import "./styles.css";

export const metadata: Metadata = {
  title: "Veil RFQ | Private RWA execution",
  description: "Sealed RFQs for permissioned Token-2022 assets on Solana."
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return <html lang="en"><body>{children}</body></html>;
}
