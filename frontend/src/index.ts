enum Status {
  UNKNOWN = "unknown",
  IDLE = "idle",
  BUSY = "busy",
  ERROR = "error",
}

interface StatusResponse {
  status: Status;
  message: string;
}

const fetchStatus = async (): Promise<StatusResponse | null> => {
  let status: StatusResponse | null = null;
  try {
    const response = await fetch("status");
    if (response.ok) {
      status = await response.json();
    }
  } catch (error: unknown) {
    console.warn(error);
  }
  return status;
};

const updateStatus = async () => {
  const statusvalue = document.getElementById("statusvalue");
  const statusbar = document.getElementsByClassName("statusbar")[0];
  if (statusvalue && statusbar) {
    const status: StatusResponse = (await fetchStatus()) ?? {
      status: Status.UNKNOWN,
      message: Status.UNKNOWN,
    };
    if (statusvalue.textContent !== status.message) {
      statusvalue.textContent = status.message;
    }
    const newClass = `status-${status.status}`;
    const oldClasses = statusbar.getAttribute("class")?.split(" ") ?? [];
    if (!oldClasses.includes(newClass)) {
      const newClasses = oldClasses
        .filter((c) => !c.startsWith("status-"))
        .concat([newClass]);
      statusbar.setAttribute("class", newClasses.join(" "));
    }
  }
};

class PollingService {
  private readonly pollingInterval: number = 2000;
  private readonly poller: () => Promise<void>;
  private abort: (() => void) | null = null;

  constructor(poller: () => Promise<void>) {
    this.poller = poller;
  }

  public setupVisibilityChangeListener() {
    document.addEventListener("visibilitychange", () => {
      if (document.hidden) {
        this.stopPolling();
      } else {
        this.startPolling();
      }
    });
  }

  public async startPolling() {
    if (!this.abort) {
      const controller = new AbortController();
      const signal = controller.signal;
      this.abort = () => {
        controller.abort();
      };

      while (!signal.aborted) {
        try {
          await this.poller();
        } catch (e: unknown) {
          console.error(e);
        }
        if (signal.aborted) {
          break;
        }
        await new Promise((resolve) =>
          setTimeout(resolve, this.pollingInterval),
        );
      }
    }
  }

  public stopPolling() {
    const abort = this.abort;
    if (abort) {
      this.abort = null;
      abort();
    }
  }
}

const pollingService = new PollingService(updateStatus);
pollingService.setupVisibilityChangeListener();
pollingService.startPolling();
