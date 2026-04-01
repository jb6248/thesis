\version "2.24.0"

\score {
  <<
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        g4 c'4 c'4 c'4 c'4 c'4 c'4 c'4 c'4 r2 r4
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        g'4 c'4 d'4 ees'4 f'4 e'4 d'4 cis'4 b4 r2 r4
      }
      \bar "|."
    }
  >>
  \layout { }
  \midi {
    \tempo 4 = 120
  }
}
