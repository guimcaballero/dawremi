# Assets folder

You can keep your audio files here.

There's two folders you should not modify: `internal` and `processed`.

## Internal

The `internal` folder keeps builtin files, like the metronome sound or the reverbs. If you remove this folder or change any of the files, the program will probably stop working!

## Processed

The `processed` folder is where the program will saved resampled sounds and any other intermediate steps, so it doesn't need to process them every time it runs. Modifying or changing any of the files should cause any issues, they'll just be regenerated the next time you run the program. Despite this, modifying this folder is not recommended.
