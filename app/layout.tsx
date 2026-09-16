import type { Metadata } from "next";
import "./styles.css";
import { Providers } from "./providers";

export const metadata: Metadata = {
  title: "Veil RFQ | Private RWA execution",
  description: "Sealed RFQs for permissioned Token-2022 assets on Solana."
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <body>
        <Providers>{children}</Providers>
      </body>
    </html>
  );
}
