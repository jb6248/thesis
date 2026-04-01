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
              r1 r1 r1 r2 aes''2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r2 b''2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 f'2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 g'2
            }
          }
          \new Voice {
            \voiceOne
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 a'2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              e''2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              r2 cis''2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              r1 r2 g'2
            }
          }
          \new Voice {
            \voiceOne
            \absolute {
              r1 r1 r1 r2 f'2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r2 e'2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 bes''2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 c''2
            }
          }
          \new Voice {
            \voiceOne
            \absolute {
              c'2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 e''2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              r1 r1 r1 r2 c''2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              g''2
            }
          }
          \new Voice {
            \voiceOne
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 c''2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 a''2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              r1 r2 bes''2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r2 aes''2
            }
          }
          \new Voice {
            \voiceOne
            \absolute {
              r2 g'2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              r2 bes''2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 d''2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              r1 r2 cis''2
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
