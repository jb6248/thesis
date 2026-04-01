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
              r1 r1 r1 r2 a''2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              c''2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r2 a''2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 fis''2
            }
          }
          \new Voice {
            \voiceOne
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 c'2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 g''2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 b'2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              r1 r1 r1 r2 c''2
            }
          }
          \new Voice {
            \voiceOne
            \absolute {
              f'2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 a''2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r2 f'2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              r2 c''2
            }
          }
          \new Voice {
            \voiceOne
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 e''2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              r2 a''2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              r1 r1 r1 r2 f'2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r2 c''2
            }
          }
          \new Voice {
            \voiceOne
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 f'2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              r1 r2 c''2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              r1 r2 f'2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              a''2
            }
          }
          \new Voice {
            \voiceOne
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 d''2
            }
          }
          \new Voice {
            \voiceTwo
            \absolute {
              r2 f'2
            }
          }
          \new Voice {
            \voiceThree
            \absolute {
              r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 c''2
            }
          }
          \new Voice {
            \voiceFour
            \absolute {
              r1 r2 a''2
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
