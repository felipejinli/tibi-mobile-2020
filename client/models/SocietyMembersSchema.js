import {ObjectId} from 'bson';

class SocietyMembers {
  constructor({partition, id = new ObjectId()}) {
    this._id = id;
    this._partition = partition;
  }

  static schema = {
    name: 'societyMembers',
    primaryKey: '_id',
    properties: {
      _id: 'objectId',
      _partition: 'string?',
      societyMembers: 'users[]?',
    },
  };
}

export default SocietyMembers;
