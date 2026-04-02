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
              r2 <c, e g>2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              r1 <aes, c ees>2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              <b, d f>2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              r1 r2 <f, c a>2
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
