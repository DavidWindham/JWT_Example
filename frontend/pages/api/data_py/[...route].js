import httpProxyMiddleware from 'next-http-proxy-middleware';

module.exports = (req, res) => {
    const { route } = req.query;
    httpProxyMiddleware(req, res, {
      target: process.env.DATA_API_ADDRESS + route,
      pathRewrite: {'^/api/data_py(/.*)?': ''},
    });
}