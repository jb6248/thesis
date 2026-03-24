\version "2.24.0"

\score {
  <<
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r4 r2 r4 c''2  r2 r4 c''2  r4
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        e,4 r2 r4 r1 r2 r4 e4
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r4 r2 r4 c'2  r2 r4 c'2  r4
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r4 c'4 e'4 g'4 r2 c'4 e'4 g'4 r2 r4
      }
      \bar "|."
    }
  >>
  \layout { }
  \midi {
    \tempo 4 = 120
  }
}
