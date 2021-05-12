import {ObjectId} from 'bson';

class EventMembers {
  constructor({partition, id = new ObjectId()}) {
    this._id = id;
    this._partition = partition;
  }

  static schema = {
    name: 'eventMembers',
    primaryKey: '_id',
    properties: {
      _id: 'objectId',
      _partition: 'string?',
      userLikes: 'users[]?',
      userSignups: 'users[]?',
    },
  };
}

export default EventMembers;
