import { PageContainer, ProCard, ProLayout } from '@ant-design/pro-components';
import { useState } from 'react';
import { Outlet, NavLink } from 'react-router';
import config from '../const/DEFAULT_LAYOUT';

const Layout = () => {
  const [pathname, setPathname] = useState('/list/sub-page/sub-sub-page1');

  return (
    <ProLayout
      splitMenus
      token={{
        colorBgAppListIconHover: 'rgba(0,0,0,0.06)',
        colorTextAppListIconHover: 'rgba(255,255,255,0.95)',
        colorTextAppListIcon: 'rgba(255,255,255,0.85)',
      }}
      {...config}
      location={{
        pathname,
      }}
      avatarProps={{
        src: 'https://gw.alipayobjects.com/zos/antfincdn/efFD%24IOql2/weixintupian_20170331104822.jpg',
        size: 'small',
        title: 'Settings',
      }}
      onMenuHeaderClick={(e) => console.log(e)}
      menuItemRender={(item, dom) => (
        <NavLink
          to={item.path || '/welcome'}
          onClick={() => {
            setPathname(item.path || '/welcome');
          }}
        >
          {dom}
        </NavLink>
      )}
      layout="top"
      fixedHeader
    >
      <PageContainer>
        <ProCard
          style={{
            height: '200vh',
            minHeight: 800,
          }}
        >
          <Outlet />
        </ProCard>
      </PageContainer>
    </ProLayout>
  );
};

export default Layout;