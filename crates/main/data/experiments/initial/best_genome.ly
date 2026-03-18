\version "2.24.0"

\score {
  <<
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r1 r2 d''2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r1 r2 g'2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r2 g'2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        g''2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        e''2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        c'2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r2 d''2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r2 b''2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r1 r2 b''2
      }
      \bar "|."
    }
  >>
  \layout { }
  \midi {
    \tempo 4 = 120
  }
}
