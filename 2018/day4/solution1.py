
def main():
    guard_notes = {}

    # Read all lines and sort them so we know that every new shift is followed
    # by all of that shift's actions
    with open("input.txt") as fh:
        lines = fh.readlines()
        lines.sort()

    # Read all log data into the following data structure
    # 'guard_id': { 'sleep_start': [...], 'awake_start': [...] }
    for line in lines:
        line = line.rstrip('\n')
        time_info, action = line.split(']')
        date, minute = time_info.split(' ')
        date = date[1:]
        action = action[1:]

        if action.startswith('Guard'):
            guard_id = action.split(' ')[1][1:]
            if guard_id not in guard_notes:
                guard_notes[guard_id] = { 'sleep_start': [], 'awake_start': [] }
        else:
            if action.startswith('falls'):
                guard_notes[guard_id]['sleep_start'].append(minute)
            else:
                guard_notes[guard_id]['awake_start'].append(minute)

    # Pass over the guard_notes and read minutes/totals into the following data
    # structure
    # 'guard_id': {
    #     'minutes': { 1: ..., 2: ..., 3: ..., ... }
    #     'total': ... }
    guard_totals = {}
    for gid in guard_notes:
        if gid not in guard_totals:
            guard_totals[gid] = { 'minutes': {}, 'total': 0 }

        for s, a in zip(guard_notes[gid]['sleep_start'], guard_notes[gid]['awake_start']):
            for i in range(int(s.split(":")[1]), int(a.split(":")[1])):
                guard_totals[gid]['total'] += 1
                if i in guard_totals[gid]['minutes']:
                    guard_totals[gid]['minutes'][i] += 1
                else:
                    guard_totals[gid]['minutes'][i] = 1

    # Get guard id who slept the most
    tmax = 0
    for gid in guard_totals:
        if guard_totals[gid]['total'] > tmax:
            guard = gid
            tmax = guard_totals[gid]['total']

    # Get the minute they slept the most at
    mmax = 0
    for m in guard_totals[guard]['minutes']:
        if guard_totals[guard]['minutes'][m] > mmax:
            mmax = guard_totals[guard]['minutes'][m]
            mout = m
    print("Guard id: {}, Minute: {}, Product: {}".format(guard, mout, int(guard) * int(mout)))

if __name__ == '__main__':
    main()

