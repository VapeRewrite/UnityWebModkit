import { LogLevel, Logger } from "../logger";
import { WebData } from "../web-data";
import { version } from "../index";

const logger = new Logger("Preloader");

export function preload(): Promise<WebData> {
  logger.info("UnityWebModkit v%s - %s", version, window.location.hostname);
  return loadWebData();
}

function loadWebData(): Promise<WebData> {
  logger.debug("Trying to load Unity web data from indexedDB cache");
  return new Promise<WebData>((resolve) => {
    indexedDB.databases().then(async (databases) => {
      logger.debug("databases:", databases);
      const unityCache = databases.findIndex((d) => d.name === "UnityCache");
      if (unityCache == -1) {
        resolve(await fallbackInterceptFetch());
        return;
      }
      console.log("UnityCache:", unityCache);

      logger.debug("Trying to open UnityCache...");
      const request = window.indexedDB.open("UnityCache");
      logger.debug("request:", request);

      request.onsuccess = async (event: any) => {
        const db = event.target.result;
        // (request.result.version === 4) -> (request.result.objectStoreNames[0] === "RequestMetaDataStore")
        logger.debug("objectStoreName:", request.result.objectStoreNames[0]);
        if (request.result.objectStoreNames[0] === "RequestMetaDataStore") {
          db.close();
          const keys = await caches.keys();

          for (const key of keys) {
            const cache = await caches.open(key);
            const requests = await cache.keys();

            for (const req of requests) {
              if (!req.url.includes(".data")) continue;

              const res = await cache.match(req);
              if (!res) continue;

              resolve(parseWebData(await res.arrayBuffer()));
              return;
            }
          }
          return;
        }
        const requestCacheEntries = db.transaction(["RequestStore"], "readonly").objectStore("RequestStore").getAll();
        logger.debug("requestCacheEntries:", requestCacheEntries);
        requestCacheEntries.onsuccess = async (event: any) => {
          const entries = event.target.result;
          if (entries.length === 0) {
            resolve(await fallbackInterceptFetch());
            return;
          }
          resolve(parseWebData(entries[0].response.parsedBody.buffer));
        };
        requestCacheEntries.onerror = async () => {
          db.close();
          resolve(await fallbackInterceptFetch());
        };
      };
    });
  });
}

async function fallbackInterceptFetch(): Promise<WebData> {
  logger.debug("Nothing in indexedDB cache, resorting to hooking Fetch API");
  return new Promise<WebData>((resolve) => {
    const originalFetch = window.fetch;
    window.fetch = async function (url) {
      logger.debug(url);
      if ((url as string).includes("webgl.data.br")) {
        window.fetch = originalFetch;
        const response = await originalFetch.apply(this, arguments as any);
        resolve(parseWebData(await response.clone().arrayBuffer()));
        return response;
      }
      return originalFetch.apply(this, arguments as any);
    };
  });
}

function parseWebData(data: ArrayBuffer): WebData {
  return new WebData(data, [["data.unity3d", 32], ["Il2CppData/Metadata/global-metadata.dat"]]);
}
