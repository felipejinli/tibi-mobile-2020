import {ObjectId} from 'bson';

const EventLineUpSchema = {
  name: 'eventLineup',
  embedded: true,
  properties: {
    time: 'date',
    description: 'string',
  },
};

class Event {
  constructor({
    partition,
    eventDescription,
    eventLineup,
    eventLocation,
    eventName,
    societyId,
    societyExclusive = false,
    ticketPrice = 0,
    discoverImages,
    id = new ObjectId(),
  }) {
    this._id = id;
    this._partition = partition;
    this.eventDescription = eventDescription;
    this.eventLineup = eventLineup;
    this.eventLocation = eventLocation;
    this.eventName = eventName;
    this.societyId = societyId;
    this.societyExclusive = societyExclusive;
    this.ticketPrice = ticketPrice;
    this.discoverImages = discoverImages;
  }

  static schema = {
    name: 'eventMembers',
    primaryKey: '_id',
    properties: {
      _id: 'objectId',
      _partition: 'string?',
      eventDescription: 'string',
      eventLineup: 'eventLineup[]',
      eventLocation: 'string',
      eventName: 'string',
      eventSpeakers: 'string[]?',
      societyId: 'societies',
      societyExclusive: 'bool',
      ticketPrice: 'float',
      discoverImages: 'string[]',
      descriptionImages: 'string[]?',
    },
  };
}

export default Event;
