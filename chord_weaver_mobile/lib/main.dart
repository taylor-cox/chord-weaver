import 'dart:convert';
import 'dart:io';
import 'dart:math';

import 'package:flutter/material.dart';
// import 'package:flutter/rendering.dart';
// import 'package:flutter/widgets.dart';
import 'package:http/http.dart' as http;

// Very, very ew.
class MyHttpOverrides extends HttpOverrides {
  @override
  HttpClient createHttpClient(SecurityContext? context) {
    return super.createHttpClient(context)
          ..badCertificateCallback = (X509Certificate cert, String host, int port) {
            //add your certificate verification logic here
            return true;
          };
  }
}

void main() {
  HttpOverrides.global = new MyHttpOverrides();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Chord Weaver',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
            seedColor: const Color.fromARGB(255, 143, 74, 0)),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Chord Weaver'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});
  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

// ew.
class SongDetails {
  final String song_name;
  final String artist;
  final int num_chords;
  final String genre;
  final String chord_roman_numeral;

  const SongDetails({
    required this.song_name,
    required this.artist,
    required this.num_chords,
    required this.genre,
    required this.chord_roman_numeral,
  });

  factory SongDetails.fromJson(Map<String, dynamic> json) {
    return switch (json) {
      {
        'song_name': String songName,
        'artist': String songArtist,
        'num_chords': int numChords,
        'genre': String songGenre,
        'chord_roman_numeral': String chordProgression,
      } =>
        SongDetails(
          song_name: songName,
          artist: songArtist,
          num_chords: numChords,
          genre: songGenre,
          chord_roman_numeral: chordProgression,
        ),
      _ => throw const FormatException('Failed to load album.'),
    };
  }
}

// ew.
class _MyHomePageState extends State<MyHomePage> {
  SongDetails songDetails = const SongDetails(
    song_name: "Ghostbusters",
    artist: "Ray Parker Jr.",
    num_chords: 7,
    genre: "Pop/Rock",
    chord_roman_numeral: "I-i-i7-i(maj7)-IV-IV7-m7",
  );
  final String API_URL = 'http://10.0.2.2:3000';

  void _shuffleSong() async {
    // Query the API for a random song.
    int randomSongId = Random().nextInt(400) + 1;
    final response = await http
        .get(Uri.parse('$API_URL/chord-progression?id=$randomSongId'));
    print(response.body.substring(1, response.body.length - 1));
    if (response.statusCode == 200) {
      setState(
        () {
          songDetails = SongDetails.fromJson(
              jsonDecode(response.body.substring(1, response.body.length - 1)) as Map<String, dynamic>
          );
        },
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        appBar: AppBar(
          title: const Text('Chord Weaver'),
          backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        ),
        body: ListView(
          children: <Widget>[
            _buildCarousel(context, 0),
          ],
        ));
  }

  Widget _buildCarousel(BuildContext context, int carouselIndex) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      children: <Widget>[
        SizedBox(
            height: 200,
            child: PageView(
              children: <Widget>[
                _buildCarouselItem(
                    context, "song_name", "Song", songDetails.song_name),
                _buildCarouselItem(
                    context, "song_artist", "Artist", songDetails.artist),
                _buildCarouselItem(context, "num_chords", "# Chords",
                    songDetails.num_chords.toString()),
                _buildCarouselItem(
                    context, "song_genre", "Genre", songDetails.genre),
              ],
            )),
        SizedBox(
            height: 200,
            child: Column(
              children: [
                const Text('Chord Progression',
                    style: TextStyle(
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                    )),
                Text(
                  songDetails.chord_roman_numeral,
                  style: const TextStyle(
                    fontSize: 24,
                  ),
                ),
              ],
            )),
        TextButton(
          onPressed: _shuffleSong,
          style: TextButton.styleFrom(
              foregroundColor: const Color.fromARGB(255, 255, 255, 255),
              backgroundColor: const Color.fromARGB(255, 159, 125, 0),
              surfaceTintColor: const Color.fromARGB(255, 79, 79, 79),
              fixedSize: const Size(75, 75)),
          child: const Text('Shuffle'),
        ),
      ],
    );
  }

  Widget _buildCarouselItem(
      BuildContext context, String keyName, String title, String subtitle) {
    return Container(
        padding: const EdgeInsets.symmetric(horizontal: 4.0),
        child: Container(
          decoration: const BoxDecoration(
            color: Color.fromARGB(255, 215, 176, 20),
            borderRadius: BorderRadius.all(Radius.circular(4.0)),
          ),
          child: Column(
            children: <Widget>[
              const Padding(padding: EdgeInsets.all(8.0)),
              Text(
                title,
                style: const TextStyle(
                  fontSize: 24,
                  fontWeight: FontWeight.bold,
                ),
              ),
              Center( child: Text(
                subtitle,
                key: Key(keyName),
                style: const TextStyle(
                  fontSize: 20,
                ),
              )),
            ],
          ),
        ));
  }
}
