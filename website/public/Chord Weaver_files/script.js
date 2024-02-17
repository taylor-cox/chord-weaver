// Create Spotify iFrame.
window.onSpotifyIframeApiReady = (IFrameAPI) => {
  const element = document.getElementById("embed-iframe");
  const options = {
    uri: "spotify:episode:7makk4oTQel546B0PZlDM5",
  };
  const callback = (EmbedController) => {
    let elem = document.getElementsByTagName("iframe")[0];
    elem.setAttribute("width", "100%");
    elem.setAttribute("height", "100%");
  };
  IFrameAPI.createController(element, options, callback);
};
