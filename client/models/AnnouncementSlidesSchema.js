class AnnouncementSlide {
  constructor({
    id,
    partition,
    announcementImage,
    announcementSubtitle,
    announcementTitle,
    urlLink,
  }) {
    this._id = id;
    this._partition = partition;
    this.announcementImage = announcementImage;
    this.announcementSubtitle = announcementSubtitle;
    this.announcementTitle = announcementTitle;
    this.urlLink = urlLink;
  }

  static schema = {
    name: 'announcementSlides',
    primaryKey: '_id',
    properties: {
      _id: 'int',
      _partition: 'string?',
      announcementImage: 'string',
      announcementSubtitle: 'string?',
      announcementTitle: 'string',
      urlLink: 'string?',
    },
  };
}

export default AnnouncementSlide;
