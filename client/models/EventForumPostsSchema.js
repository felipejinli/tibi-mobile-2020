import {ObjectId} from 'bson';

const CommentSchema = {
  name: 'comment',
  embedded: true,
  properties: {
    userId: 'users',
    commentText: 'string',
    dateTime: 'date',
    upvotes: 'int',
  },
};

class EventForumPost {
  constructor({
    partition,
    eventId,
    userId,
    postText,
    dateTimePosted,
    postTheme = EventForumPost.THEME_DEFAULT,
    id = new ObjectId(),
  }) {
    this._id = id;
    this._partition = partition;
    this.eventId = eventId;
    this.userId = userId;
    this.postText = postText;
    this.dateTimePosted = dateTimePosted;
    this.postTheme = postTheme;
  }

  static THEME_DEFAULT = 'Default';
  static THEME_DIVERSITY = 'Diversity';
  static THEME_FEELING = 'Feeling';

  static schema = {
    name: 'eventForumPosts',
    primaryKey: '_id',
    properties: {
      _id: 'objectId',
      _partition: 'string?',
      eventId: 'events',
      userId: 'users',
      comments: 'comment[]?',
      postText: 'string',
      upvotes: {type: 'int', default: 0},
      dateTimePosted: 'date',
      postTheme: 'string',
    },
  };
}

export default EventForumPost;
