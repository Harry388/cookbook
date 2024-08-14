package handlers

import (
	"cookbook/templates"
	"cookbook/templates/user"
	"cookbook/templates/util"
	"net/http"

	"github.com/labstack/echo/v5"
	"github.com/pocketbase/dbx"
	"github.com/pocketbase/pocketbase"
	"github.com/pocketbase/pocketbase/apis"
	"github.com/pocketbase/pocketbase/forms"
	"github.com/pocketbase/pocketbase/models"
)

type userProfileInfo struct {
    followers int
    following int
    self bool
    isFollowing bool
}

func getProfileInfo(app *pocketbase.PocketBase, record *models.Record, auth *models.Record) userProfileInfo {
    type value struct { Value int }

    followers := value{}
    following := value{}

    app.Dao().DB().
        Select("count(*) as value").
        From("following").
        Where(dbx.NewExp("following = {:id}", dbx.Params{ "id": record.Id })).
        One(&followers)

    app.Dao().DB().
        Select("count(*) as value").
        From("following").
        Where(dbx.NewExp("user = {:id}", dbx.Params{ "id": record.Id })).
        One(&following)

    self := (auth != nil) && (auth.Username() == record.Username())
    isFollowing := false

    if (!self) && (auth != nil) {
        result := value{}
        app.Dao().DB().
            Select("count(*) as value").
            From("following").
            Where(dbx.NewExp("following = {:id}", dbx.Params{ "id": record.Id })).
            Where(dbx.NewExp("user = {:id}", dbx.Params{ "id": auth.Id })).
            One(&result)
        if result.Value == 1{
            isFollowing = true
        }
    }

    return userProfileInfo {
        followers: followers.Value,
        following: following.Value,
        self: self,
        isFollowing: isFollowing,
    }
}

func (h *handler) profilePage(c echo.Context) error {
    record, _ := c.Get(apis.ContextAuthRecordKey).(*models.Record)
    info := getProfileInfo(h.app, record, record)
    t := user.UserPage(record, info.followers, info.following, info.self, info.isFollowing)
    return templates.Render(t, c)
}

func (h *handler) userPage(c echo.Context) error {
    username := c.PathParam("username")
    if username == "" {
        return echo.NewHTTPError(http.StatusBadRequest)
    }

    record, err := h.app.Dao().FindAuthRecordByUsername("users", username)
    if err != nil {
        return echo.NewHTTPError(http.StatusNotFound)
    }

    auth, _ := c.Get(apis.ContextAuthRecordKey).(*models.Record)

    info := getProfileInfo(h.app, record, auth)
    t := user.UserPage(record, info.followers, info.following, info.self, info.isFollowing)
    return templates.Render(t, c)
}

func (h *handler) userFollowUser(c echo.Context) error {
    username := c.PathParam("username")
    if username == "" {
        return echo.NewHTTPError(http.StatusBadRequest)
    }

    record, err := h.app.Dao().FindAuthRecordByUsername("users", username)
    if err != nil {
        return echo.NewHTTPError(http.StatusNotFound)
    }

    auth, _ := c.Get(apis.ContextAuthRecordKey).(*models.Record)

    following, err := h.app.Dao().FindCollectionByNameOrId("following")
    if err != nil {
        return echo.NewHTTPError(http.StatusInternalServerError)
    }

    followingRecord := models.NewRecord(following)
    followingRecord.Set("user", auth.Id)
    followingRecord.Set("following", record.Id)

    if err := h.app.Dao().SaveRecord(followingRecord); err != nil {
        return echo.NewHTTPError(http.StatusInternalServerError)
    }

    info := getProfileInfo(h.app, record, auth)
    t := user.FollowResponse(record, info.followers, info.following, info.self, info.isFollowing)
    return templates.Render(t, c)
}

func (h *handler) userUnfollowUser(c echo.Context) error {
    username := c.PathParam("username")
    if username == "" {
        return echo.NewHTTPError(http.StatusBadRequest)
    }

    record, err := h.app.Dao().FindAuthRecordByUsername("users", username)
    if err != nil {
        return echo.NewHTTPError(http.StatusNotFound)
    }

    auth, _ := c.Get(apis.ContextAuthRecordKey).(*models.Record)

    followingRecord, err := h.app.Dao().FindRecordsByExpr("following", dbx.HashExp{ "user": auth.Id, "following": record.Id })
    if (err != nil) || (len(followingRecord) == 0) {
        return echo.NewHTTPError(http.StatusNotFound)
    }

    if err := h.app.Dao().DeleteRecord(followingRecord[0]); err != nil {
        return echo.NewHTTPError(http.StatusInternalServerError)
    }

    info := getProfileInfo(h.app, record, auth)
    t := user.FollowResponse(record, info.followers, info.following, info.self, info.isFollowing)
    return templates.Render(t, c)
}

func (h *handler) userAvatar(c echo.Context) error {
    username := c.PathParam("username")
    if username == "" {
        return echo.NewHTTPError(http.StatusBadRequest)
    }

    record, err := h.app.Dao().FindAuthRecordByUsername("users", username)
    if err != nil {
        return echo.NewHTTPError(http.StatusNotFound)
    }

    key := record.BaseFilesPath() + "/" + record.GetString("avatar")

    fsys, _ := h.app.NewFilesystem()
    defer fsys.Close()

    blob, _ := fsys.GetFile(key)
    defer blob.Close()

    _, err = blob.WriteTo(c.Response())
    return err
}

func (h *handler) updateUserAvatar(c echo.Context) error {
    record, _ := c.Get(apis.ContextAuthRecordKey).(*models.Record)

    form := forms.NewRecordUpsert(h.app, record)
    form.LoadRequest(c.Request(), "")

    if err := form.Submit(); err != nil {
        t := util.ErrAlert(err.Error())
        return templates.Render(t, c)
    }

    return nil
}

func (h *handler) updateProfile(c echo.Context) error {
    type updateProfile struct {
        Username string `form:"username"`
        Name string `form:"name"`
        Bio *string `form:"bio"`
    }

    record, _ := c.Get(apis.ContextAuthRecordKey).(*models.Record)

    var update updateProfile
    if err := c.Bind(&update); err != nil {
        c.Response().Status = 400;
        t := util.ErrAlert("Invalid Value")
        return templates.Render(t, c)
    }

    if update.Bio != nil {
        record.Set("bio", *update.Bio)
    }

    if update.Name != "" {
        record.Set("name", update.Name)
    }

    if update.Username != "" {
        record.SetUsername(update.Username)
    }

    if err := h.app.Dao().SaveRecord(record); err != nil {
        if update.Username != "" {
            c.Response().Status = 400;
            t := util.ErrAlert("Username is taken")
            return templates.Render(t, c)
        }
        c.Response().Status = 400;
        t := util.ErrAlert("Invalid Value")
        return templates.Render(t, c)
    }

    return nil
}
