\version "2.24.0"

\score {
  <<
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        c'4 d'4 e'4 f'4 g'4 a'4 b'4 c''4 b'4 a'4 g'4 f'4 e'4 d'4 c'2
      }
      \bar "|."
    }
  >>
  \layout { }
  \midi {
    \tempo 4 = 120
  }
}
