\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        <<
          \new Voice {
            \voiceOne
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 e''2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 d''2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              bes''2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              r1 r2 g''2
            }
          }
          \new Voice {
            \voiceOne
            \absolute {
              r1 r2 b''2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 e''2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 cis''2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              d''2
            }
          }
          \new Voice {
            \voiceOne
            \absolute {
              r2 a'2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              r2 e''2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              r1 r2 e'2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              r1 r1 r1 r2 d''2
            }
          }
          \new Voice {
            \voiceOne
            \absolute {
              r1 r1 r1 r2 bes''2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 a'2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 cis''2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              r1 r1 r1 r2 g'2
            }
          }
          \new Voice {
            \voiceOne
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r2 cis''2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r2 a'2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 g'2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              r2 cis''2
            }
          }
          \new Voice {
            \voiceOne
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r2 e''2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              g'2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 a'2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 bes''2
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
