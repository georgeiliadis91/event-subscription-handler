class liveSubscriptionHandler {
  constructor() {
    this.subScriptions = {};
    this.store = {
      isBusy: false,
    };
  }

  #isStoreReady() {
    return new Promise((resolve, reject) => {
      try {
        if (this.store.isBusy) {
          reject(new Error("STORE_BUSY"));
        } else {
          this.store.isBusy = true;
          resolve();
        }
      } catch (err) {
        reject(err);
      }
    });
  }

  addItems(itemIds = []) {
    this.#isStoreReady()
      .then(
        () => {
          //  *** slow async process with a remote ****
          //Register items with remote over socket so the get updated,,
        },
        (rejection) => {
          if (rejection.message === "STORE_BUSY") {
            this.addItems(itemIds);
          }
          throw new Error(rejection);
        }
      )
      .catch((err) => {
        console.error(err);
      });
  }

  removeItems(itemIds = []) {
    this.#isStoreReady()
      .then(
        () => {
          //  *** slow async process with a remote ****
          //Unregister items with remote over socket so the get updated,,
        },
        (rejection) => {
          if (rejection.message === "STORE_BUSY") {
            this.removeItems(itemIds);
          }
          throw new Error(rejection);
        }
      )
      .catch((err) => {
        console.error(err);
      });
  }
}

const liveSubscriptionUpdater = new liveSubscriptionHandler();
