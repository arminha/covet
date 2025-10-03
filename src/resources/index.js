// src/index.ts
var fetchStatus = async () => {
  let status = {
    status: "unknown" /* UNKNOWN */,
    message: "unknown" /* UNKNOWN */
  };
  try {
    const response = await fetch("status");
    if (response.ok) {
      status = await response.json();
    }
  } catch (error) {
    if (error instanceof Error) {
      if (error.name === "TypeError" && error.message.includes("NetworkError")) {
        console.debug("Ignore NetworkError", error);
        return null;
      }
    }
    console.warn(error);
  }
  return status;
};
var updateStatus = async () => {
  const statusvalue = document.getElementById("statusvalue");
  const statusbar = document.getElementsByClassName("statusbar")[0];
  if (statusvalue && statusbar) {
    const status = await fetchStatus();
    if (status === null) {
      return;
    }
    if (statusvalue.textContent !== status.message) {
      statusvalue.textContent = status.message;
    }
    const newClass = `status-${status.status}`;
    const oldClasses = statusbar.getAttribute("class")?.split(" ") ?? [];
    if (!oldClasses.includes(newClass)) {
      const newClasses = oldClasses.filter((c) => !c.startsWith("status-")).concat([newClass]);
      statusbar.setAttribute("class", newClasses.join(" "));
    }
  }
};

class PollingService {
  pollingInterval = 2000;
  poller;
  abort = null;
  constructor(poller) {
    this.poller = poller;
  }
  setupVisibilityChangeListener() {
    document.addEventListener("visibilitychange", () => {
      if (document.hidden) {
        this.stopPolling();
      } else {
        this.startPolling();
      }
    });
  }
  async startPolling() {
    if (!this.abort) {
      const controller = new AbortController;
      const signal = controller.signal;
      this.abort = () => {
        controller.abort();
      };
      while (!signal.aborted) {
        try {
          await this.poller();
        } catch (e) {
          console.error(e);
        }
        if (signal.aborted) {
          break;
        }
        await new Promise((resolve) => setTimeout(resolve, this.pollingInterval));
      }
    }
  }
  stopPolling() {
    const abort = this.abort;
    if (abort) {
      this.abort = null;
      abort();
    }
  }
}
var pollingService = new PollingService(updateStatus);
pollingService.setupVisibilityChangeListener();
pollingService.startPolling();

//# debugId=449D4A1B30B7E06964756E2164756E21
