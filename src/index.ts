import dotenv from "dotenv";
import { Telegraf } from "telegraf";
dotenv.config();

const bot = new Telegraf(process.env.BOT_TOKEN!);

bot.command("launch", async () => {
  const res = await fetch("http://localhost:3000/launch", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      token_name: "Test Token",
      symbol: "TEST",
      decimals: 9,
    }),
  });

  const data = await res.json();
  console.log("[Rust Response]:", data);
});

bot.start((ctx) => ctx.reply("Pump.fun bot ready!"));

bot.launch();
