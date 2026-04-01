\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        r1
        \bar "|."
      }
      \new Staff {
        \clef bass
        \time 4/4
        <<
          \new Voice {
            \voiceOne
            \absolute {
              e2 r2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              g2 r2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              c2 r2
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
