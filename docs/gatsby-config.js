
module.exports = {
  siteMetadata: {
    title: `Shelfdb`,
    description: `The GraphQL Database!`,
    author: `Joatin Granlund <granlundjoatin@icloud.com>`,
    repository: `https://github.com/Joatin/shelfdb`,
    menuLinks:[
      {
        name:'Docs',
        link:'/docs',
        icon: 'book'
      },
      {
        name:'Blog',
        link:'/blog',
        icon: 'rss'
      }
    ]
  },
  plugins: [
    `gatsby-plugin-react-helmet`,
    {
      resolve: `gatsby-source-filesystem`,
      options: {
        name: `images`,
        path: `${__dirname}/src/images`,
      },
    },
    `gatsby-transformer-sharp`,
    `gatsby-plugin-sharp`,
    `gatsby-plugin-typescript`,
    `gatsby-plugin-sass`,
    {
      resolve: `gatsby-plugin-manifest`,
      options: {
        name: `shelf`,
        short_name: `shelf`,
        start_url: `/`,
        background_color: `#663399`,
        theme_color: `#663399`,
        display: `minimal-ui`,
        icon: `src/images/gatsby-icon.png`, // This path is relative to the root of the site.
      },
    },
    `gatsby-plugin-mdx`,
    {
      resolve: `gatsby-source-filesystem`,
      options: {
        name: `docs`,
        path: `${__dirname}/src/docs`,
      },
    },
    `gatsby-plugin-netlify`,
    `gatsby-plugin-offline`,
    {
      resolve: `gatsby-plugin-google-analytics`,
      options: {
        trackingId: "UA-159442960-1",
        head: false,
      },
    }
  ],
};
