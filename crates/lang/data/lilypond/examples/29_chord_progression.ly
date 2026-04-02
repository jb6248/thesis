\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef bass
        \time 4/4
        <<
          \new Voice {
            \voiceOne
            \absolute {
              r1 r2 g2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              d2 r2 r1
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              bes,2 r2 r1
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              r1 r2 e2
            }
          }
          \new Voice {
            \voiceOne
            \absolute {
              r2 e2 r1
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              r2 g2 r1
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              r1 r2 cis,2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              r2 cis,2 r1
            }
          }
          \new Voice {
            \voiceOne
            \absolute {
              f2 r2 r1
            }
          }
        >>
        \bar "|."
      }
    >>
  >>
  \layout { }
  \midi {
    \tempo 4 = 120
  }
}
