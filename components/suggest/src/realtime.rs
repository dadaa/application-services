/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use crate::{
    db::{KeywordInsertStatement, SuggestDao, SuggestionInsertStatement},
    rs::{DownloadedRealtimeGroupSuggestion, DownloadedRealtimeSuggestion, SuggestRecordId},
    suggestion::Suggestion,
    Result, SuggestionProvider, SuggestionQuery,
};
use rusqlite::named_params;
use sql_support::ConnExt;

impl SuggestDao<'_> {
    /// Inserts the suggestions for Realtime attachment into the database.
    pub(crate) fn insert_realtime_suggestions(
        &mut self,
        record_id: &SuggestRecordId,
        suggestions: &[DownloadedRealtimeSuggestion],
    ) -> Result<()> {
        let mut suggestion_insert = SuggestionInsertStatement::new(self.conn)?;
        let mut keyword_insert = KeywordInsertStatement::new(self.conn)?;
        for suggestion in suggestions {
            let suggestion_id = suggestion_insert.execute(
                record_id,
                &suggestion.title,
                &suggestion.url,
                suggestion.score,
                SuggestionProvider::Realtime,
            )?;
            for (rank, keyword) in suggestion.keywords.iter().enumerate() {
                keyword_insert.execute(suggestion_id, keyword, None, rank)?;
            }
            self.scope.err_if_interrupted()?;
            self.conn.execute_cached(
                "INSERT INTO realtime_custom_details(suggestion_id, category, id, description)
                 VALUES(:suggestion_id, :category, :id, :description)",
                named_params! {
                    ":suggestion_id": suggestion_id,
                    ":category": suggestion.category.as_str(),
                    ":id": suggestion.id.as_str(),
                    ":description": suggestion.description.as_str(),
                },
            )?;
        }
        Ok(())
    }

    /// Inserts the suggestions for RealtimeGroup attachment into the database.
    pub(crate) fn insert_realtime_group_suggestions(
        &mut self,
        _record_id: &SuggestRecordId,
        _suggestion: &[DownloadedRealtimeGroupSuggestion],
    ) -> Result<()> {
        Ok(())
    }

    /// Fetch Realtime suggestion from given user's query.
    pub(crate) fn fetch_realtime_suggestions(
        &self,
        _query: &SuggestionQuery,
    ) -> Result<Vec<Suggestion>> {
        Ok(vec![])
    }

    /// Fetch RealtimeGroup suggestion from given user's query.
    pub(crate) fn fetch_realtime_group_suggestions(
        &self,
        _query: &SuggestionQuery,
    ) -> Result<Vec<Suggestion>> {
        Ok(vec![])
    }
}
