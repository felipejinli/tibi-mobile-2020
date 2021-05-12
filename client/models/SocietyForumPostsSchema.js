import {ObjectId} from 'bson';

class SocietyForumPost {
  constructor({
    partition,
    societyId,
    userId,
    postText,
    dateTimePosted,
    postTheme = SocietyForumPost.THEME_DEFAULT,
    id = new ObjectId(),
  }) {
    this._id = id;
    this._partition = partition;
    this.societyId = societyId;
    this.userId = userId;
    this.postText = postText;
    this.dateTimePosted = dateTimePosted;
    this.postTheme = postTheme;
  }

  static THEME_DEFAULT = 'Default';
  static THEME_DIVERSITY = 'Diversity';
  static THEME_FEELING = 'Feeling';

  static schema = {
    name: 'societyForumPosts',
    primaryKey: '_id',
    properties: {
      _id: 'objectId',
      _partition: 'string?',
      societyId: 'societies',
      userId: 'users',
      postText: 'string',
      comments: 'comment[]?',
      upVotes: {type: 'int', default: 0},
      postTheme: 'string',
      dateTimePosted: 'date',
    },
  };
}

export default SocietyForumPost;
