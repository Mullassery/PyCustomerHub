import { describe, expect, it } from "vitest";
import { PyCustomerHubWebAnalytics } from "../src/client.js";
import { InMemoryStorage } from "../src/memory-storage.js";

describe("PyCustomerHubWebAnalytics", () => {
  it("creates and persists an anonymous id", () => {
    const storage = new InMemoryStorage();

    const analytics = new PyCustomerHubWebAnalytics({
      writeKey: "tenant-1",
      apiUrl: "http://localhost:8080",
      storage,
      flushIntervalMs: 0,
      fetchImpl: async () => new Response(null, { status: 200 }),
    });
    analytics.destroy();

    expect(storage.getItem("pycustomerhub:anonymous_id")).toMatch(/^anon-/);
  });
});
