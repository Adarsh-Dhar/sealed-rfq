import { createClient } from "@solana/kit";
import { walletPlugin } from "@solana/kit-plugin-wallet";

// Devnet for now — this should track whatever cluster you deployed
// veil_rfq to in the deployment guide.
export const solanaClient = createClient({
  urlOrMoniker: "devnet",
}).use(walletPlugin());
