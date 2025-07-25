import { Keypair } from "@solana/web3.js";
import fs from "fs";

const keypair = Keypair.generate();
const secretKeyArray = Array.from(keypair.secretKey);
const publicKey = keypair.publicKey.toBase58();

console.log("✅ Generated !");
console.log("Public address :", publicKey);
console.log("Private key :", secretKeyArray.join(","));

fs.writeFileSync(
  "wallet.json",
  JSON.stringify(
    {
      publicKey,
      secretKey: secretKeyArray,
    },
    null,
    2
  )
);
