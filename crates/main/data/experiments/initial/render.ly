\version "2.24.0"

\score {
  <<
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r1 r1 r1 r2 aes''2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r1 r1 r1 r2 b''2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r1 r2 e''2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r1 r2 a'2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        fis'2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r2 c''2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r1 r1 r1 r2 e'2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r2 f'2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        a''2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r2 a''2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        r1 r2 c''2
      }
      \bar "|."
    }
    \new Staff {
      \set Staff.instrumentName = "Piano"
      \time 4/4
      \absolute {
        c''2
      }
      \bar "|."
    }
  >>
  \layout { }
  \midi {
    \tempo 4 = 120
  }
}
