\version "2.24.0"

\score {
  <<
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        c'4 e'4 g'4 c''4~c''4 r2 r4
      }
      \bar "|."
    }
  >>
  \layout { }
  \midi {
    \tempo 4 = 120
  }
}
