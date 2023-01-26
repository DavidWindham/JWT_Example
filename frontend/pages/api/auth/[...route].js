import httpProxyMiddleware from 'next-http-proxy-middleware';


module.exports = (req, res) => {
    const { route } = req.query;
    httpProxyMiddleware(req, res, {
      target: process.env.AUTH_API_ADDRESS + route,
      pathRewrite: {'^/api/auth(/.*)?': ''},
      headers: {
        'x-api-key': process.env.AWS_GATEWAY_API_KEY
      }
    });
}