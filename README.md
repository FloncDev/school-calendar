# School Calendar
A web server that takes in my schedule as a json file and returns it in .ical format. Used to show my class schedule on my calendar.

## How to use
Currently it is not in a usable state for the public but if you want you could try. Create a file called `schedule.json` in the same directory as this README.md file. Inside, copy the following format
_the following is out of date, json structure has been updated. TODO: change
```json
[
    [
        {
            "name": "Class name",
            "classroom": "F18",
            "start_time": "10:00 AM",
            "end_time": "11:00 AM"
        }
    ]
]
```
The first array is for the days, and the second is for the classes in that day. It doesn't only have to be classes though - but planning to implement a dedicated feature for clubs and revision lessons.

### Todo
- [ ] Add term dates
    - This also means not generating calendar events during the holidays
- [ ] Add homework due
- [ ] Add integration with Edulink ical export
    - Edulink is the software my school uses, it supports export to ICAL but only for the next week.
- [ ] Add school events
- [ ] Add period 7 and 8 + clubs
    - After school extra lessons / clubs. Could change on a weekly basis
    - Could also mean adding a scheduler to make sure I go to everything I want to equally
- [x] Dockerize
