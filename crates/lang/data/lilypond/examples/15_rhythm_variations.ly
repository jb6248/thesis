\version "2.24.0"

\score {
  <<
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        c'1 e'1 g'1 c'2 d'2 e'2 f'2 c'4 d'4 e'4 f'4 g'4 a'4 b'4 c''4 c'2 e'4 g'4 c''1
      }
      \bar "|."
    }
  >>
  \layout { }
  \midi {
    \tempo 4 = 120
  }
}
