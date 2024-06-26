import dotenv from "dotenv";
import path from "path";
import fs from "fs";

export function rootPath(dir: string) {
  return path.resolve(__dirname, "../../", dir);
}

const envPath = rootPath("./config.env");

if (fs.existsSync(envPath)) {
  dotenv.config({ path: envPath });
}
const e = process.env as { [key: string]: string };

export const IS_PRODUCTION = e.IS_PRODUCTION === "true",
  PATH = {
    TO_STATIC: e.PATH_TO_STATIC_FROM_ROOT_DIR,
    TO_ENCRYPTION_KEY: rootPath(e.PATH_TO_ENCRYPTION_KEY),
    TO_TEST_WALLETS: rootPath(e.PATH_TO_TEST_WALLETS),
    TO_TEST_WALLETS_PUBLIC: rootPath(e.PATH_TO_TEST_WALLETS_PUBLIC),
  },
  PORT = e.PORT,
  BASE_URL = {
    DEV: `${e.BASE_URL_DEV}:${e.PORT}`,
    PROD: e.BASE_URL_PROD,
    PROXY: e.BASE_URL_PROXY,
  },
  CHAIN_TYPE = e.CHAIN_TYPE as "main" | "test",
  DAPP_ADDRESS = e.DAPP_ADDRESS;
