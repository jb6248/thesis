\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          c'4 d'4 e'4 f'4
        }
        \bar "|."
      }
      \new Staff {
        \clef bass
        \time 4/4
        \absolute {
          c2 g2
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
