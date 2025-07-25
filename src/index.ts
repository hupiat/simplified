import dotenv from "dotenv";
import { Telegraf } from "telegraf";
dotenv.config();

const bot = new Telegraf(process.env.BOT_TOKEN!);

bot.start((ctx) => ctx.reply("Pump.fun bot ready!"));

bot.launch();
