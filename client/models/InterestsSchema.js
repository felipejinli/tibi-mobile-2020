class Interest {
  constructor({id, partition, interestImage}) {
    this._id = id;
    this._partition = partition;
    this.interestImage = interestImage;
  }

  static schema = {
    name: 'interests',
    primaryKey: '_id',
    properties: {
      _id: 'string',
      _partition: 'string?',
      interestImage: 'string',
    },
  };
}

export default Interest;
