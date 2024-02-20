// Create Spotify iFrame.
window.onSpotifyIframeApiReady = (IFrameAPI) => {
  const element = document.getElementById("embed-iframe");
  const options = {
    uri: "spotify:track:3m0y8qLoznUYi73SUBP8GI",
  };
  const callback = (EmbedController) => {
    let elem = document.getElementsByTagName("iframe")[0];
    elem.setAttribute("width", "100%");
    elem.setAttribute("height", "100%");
  };
  IFrameAPI.createController(element, options, callback);
};

// Adds functionality to shuffle button
function add_shuffle_button_functionality() {
  $('#shuffle-button').click(() => {
    // Generate a random number between 1 and 401.
    let randomProgressionID = Math.floor(Math.random() * 401) + 1;
    console.info("Getting chord progression with id = " + randomProgressionID);

    // Fetch content from API.
    fetch('http://localhost:3000/chord-progression?id=' + randomProgressionID)
    .then(response => response.json())
    .then(populate_page_with_data)
    .catch((error) => {
      console.error('Error:', error);
    });
  });
}

// Populates page with data from API, and updates the spotify iframe.
function populate_page_with_data(data) {
  data = data[0];
  console.log(data.chord_roman_numeral.replaceAll(', ', '-'));
  $("#p-song-name").text(data.song_name);
  $("#p-song-artist").text(data.artist);
  $("#p-num-chords").text(data.num_chords);
  $("#p-genre").text(data.genre);
  $("#p-chord-progression").text(data.chord_roman_numeral.replaceAll(', ', '-'));

  // Update spotify iframe.
  let cleaned_song_name = encodeURIComponent(
    data.song_name
      .replaceAll(/ *\([^)]*\) */g, "")
  );
  let cleaned_artist_name = encodeURIComponent(
    data.artist
  );

  let search_term = cleaned_song_name + "%20" + cleaned_artist_name;

  fetch('http://localhost:3000/spotify?search=' + search_term)
  .then(response => response.json())
  .then((data) => {
    console.log(data);
    let songURI = data.uri;
    let elem = document.getElementsByTagName("iframe")[0];
    elem.setAttribute("src", "https://open.spotify.com/embed/track/" + songURI.split(":")[2]);
  })
}

// Main function
function main() {
  add_shuffle_button_functionality();
}

// Run main on page load.
window.onload = main;