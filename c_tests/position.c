#include <assert.h>
#include <shogi_core.h>
#include <stdio.h>
#include <string.h>

int main(void) {
    PartialPosition pos;
    PartialPosition_startpos(&pos);
    char buf[200];
    PartialPosition_to_sfen_c(&pos, (uint8_t *) buf);
    assert (strcmp(buf, "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL B - 1") == 0);

    memset(buf, 0, sizeof(buf));

    Position *p = Position_startpos();
    Position_to_sfen_c(p, (uint8_t *) buf);
    Position_destruct(p);
    assert (strcmp(buf, "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL B - 1") == 0);
    return 0;
}
