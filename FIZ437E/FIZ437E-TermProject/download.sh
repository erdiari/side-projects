#!/bin/bash

# yt-dlp -x --audio-format "wav" -a music-list.txt -o 'Music/%(title)s [%(id)s].%(ext)s'
yt-dlp -x --audio-format "opus" -a music-list.txt -o 'Music/%(title)s [%(id)s].%(ext)s'
