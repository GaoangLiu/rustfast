#!/usr/bin/env python
import os
import random
import re
import sys
from collections import defaultdict

import arrow
import codefast as cf

today = arrow.Arrow.now().format('YYYY-MM-DD')
his_data = jsn.read('data/cnt.json')
if today in his_data:
    del his_data[today]
sorted_data = [(k, v) for k, v in his_data.items()]
cnt = 0
for e in io.walk('./', depth=3):
    if e.endswith('rs'):
        cnt += len(io.read(e))

his_data[today] = cnt
cf.say(his_data)

print(f'Today\'s progress: {cnt-sorted_data[-1][-1]} lines')

jsn.write(his_data, 'data/cnt.json')
