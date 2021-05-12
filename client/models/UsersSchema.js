import {ObjectId} from 'bson';

const EventInvite = {
  name: 'eventInvite',
  embedded: true,
  properties: {
    sentUserID: 'users',
    eventId: 'events',
  },
};

const sentInvite = {
  name: 'sentInvite',
  embedded: true,
  properties: {
    UsersSentTo: 'users[]',
    eventId: 'events',
  },
};

class User {
  constructor({
    partiton,
    yearOfStudy,
    emailAddress,
    firstName,
    lastName,
    uniDepartment,
    userInterests = [],
    onlineStatus,
    id = new ObjectId(),
  }) {
    this._id = id;
    this._partition = partiton;
    this.yearOfStudy = yearOfStudy;
    this.emailAddress = emailAddress;
    this.firstName = firstName;
    this.lastName = lastName;
    this.uniDepartment = uniDepartment;
    this.userInterests = userInterests;
    this.onlineStatus = onlineStatus;
  }

  static schema = {
    name: 'users',
    primaryKey: '_id',
    properties: {
      _id: 'objectId',
      _partition: 'string?',
      onlineStatus: 'boolean',
      yearOfStudy: 'int?',
      emailAddress: 'string',
      firstName: 'string',
      lastName: 'string',
      uniDepartment: 'string',
      userDescription: 'string?',
      summaryStatement: 'string?',
      profilePictures: 'string[]?',
      userInterests: 'string[]',
      friends: 'users[]?',
      societyPosts: 'societyForumPosts[]?',
      eventPosts: 'eventForumPosts[]?',
      taggedSocietyPosts: 'societyForumPosts[]?',
      taggedEventPosts: 'eventForumPosts[]?',
      societyMemberships: 'societies[]?',
      eventsLiked: 'events[]?',
      eventsSignedUp: 'events[]?',
      eventsInvited: 'eventInvite[]?',
      eventsSentInvites: 'sentInvite[]?',
    },
  };
}

export default User;
