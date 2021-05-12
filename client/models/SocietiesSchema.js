import {ObjectId} from 'bson';

class Society {
  constructor({
    partition,
    societyName,
    relatedSocieties,
    societyLogo,
    societyImages,
    id = new ObjectId(),
  }) {
    this._partition = partition;
    this._id = id;
    this.societyName = societyName;
    this.relatedSocieties = relatedSocieties;
    this.societyLogo = societyLogo;
    this.societyImages = societyImages;
  }

  static schema = {
    name: 'societies',
    primaryKey: '_id',
    properties: {
      _id: 'objectId',
      _partition: 'string?',
      societyName: 'string',
      currentEventIds: 'events[]?',
      oldEventIds: 'events[]?',
      relatedSocieties: 'societies[]?',
      storyVideos: 'string[]?',
      societyLogo: 'string?',
      societyImages: 'string[]?',
    },
  };
}

export default Society;
