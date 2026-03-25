\version "2.24.0"

\score {
  <<
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r1 r2 a''2
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
        r1 r2 cis''2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r2 g''2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r2 bes''2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        ees'2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        bes''2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r2 ees'2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r1 r2 fis'2
      }
      \bar "|."
    }
  >>
  \layout { }
  \midi {
    \tempo 4 = 120
  }
}
