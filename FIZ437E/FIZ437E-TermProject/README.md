# MUSIC GENERATION WITH RNN

## Downloading required modules
This project uses the `pipenv` as virtual environment and package manager. So you must run the following command in a environment where the `pipenv` is available.
```bash
pipenv sync
```

## Downloading Training Music
Downloading the music is done by using cli program `yt-dlp`. And `yt-dlp` uses `ffmpeg` or `avconc` to convert videos to audio files.
If you have these programs in your system you should be able to download the training music by running the `download.sh` in any UNIX compliant shell.

## Preprocessing the training music
Preprocessing the training music is done by python script `preprocess.py` which uses `pipenv`. To run the script you must run the following command
```bash
pipenv run preprocess.py
```