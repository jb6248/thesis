\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          c'4 e'4 g'4 e'4 d'4 f'4 a'4 f'4 c'4 e'4 g'4 e'4 c'1
        }
        \bar "|."
      }
    >>
  >>
  \layout { }
  \midi {
    \tempo 4 = 120
  }
}
